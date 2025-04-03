import sys
from pyspark.sql import SparkSession

# from pyspark.sql.functions import col
import time
import os


def create_spark_session():
    """Create a minimal Spark session with S3 configuration"""
    print("Creating Spark session...")

    conf = SparkSession.builder.appName("Test pyspark read json from s3")

    # Basic configurations
    conf = conf.config("spark.driver.host", "data-pipeline-airflow")
    conf = conf.config("spark.driver.bindAddress", "0.0.0.0")

    # S3 Configuration
    conf = conf.config(
        "spark.hadoop.fs.s3a.endpoint", "http://data-pipeline-localstack:4566"
    )
    conf = conf.config("spark.hadoop.fs.s3a.access.key", "test")
    conf = conf.config("spark.hadoop.fs.s3a.secret.key", "test")
    conf = conf.config("spark.hadoop.fs.s3a.path.style.access", "true")
    conf = conf.config(
        "spark.hadoop.fs.s3a.impl", "org.apache.hadoop.fs.s3a.S3AFileSystem"
    )
    conf = conf.config(
        "spark.hadoop.fs.s3a.aws.credentials.provider",
        "org.apache.hadoop.fs.s3a.SimpleAWSCredentialsProvider",
    )

    # Debug configurations
    conf = conf.config("spark.hadoop.fs.s3a.connection.timeout", "120000")
    conf = conf.config("spark.hadoop.fs.s3a.connection.ssl.enabled", "false")
    conf = conf.config("spark.hadoop.fs.s3a.impl.disable.cache", "true")
    conf = conf.config("spark.hadoop.fs.s3a.logging.level", "DEBUG")
    conf = conf.config("spark.executor.heartbeatInterval", "30s")

    # S3A Committer configurations
    conf = conf.config(
        "spark.hadoop.mapreduce.fileoutputcommitter.algorithm.version", "2"
    )
    conf = conf.config(
        "spark.hadoop.mapreduce.fileoutputcommitter.cleanup-failures.ignored", "true"
    )
    conf = conf.config(
        "spark.sql.parquet.output.committer.class",
        "org.apache.spark.internal.io.cloud.BindingParquetOutputCommitter",
    )
    conf = conf.config("spark.hadoop.fs.s3a.committer.name", "directory")
    conf = conf.config("spark.hadoop.fs.s3a.committer.staging.conflict-mode", "replace")
    conf = conf.config(
        "spark.hadoop.fs.s3a.committer.staging.tmp.path", "/tmp/spark_staging"
    )
    conf = conf.config("spark.hadoop.fs.s3a.buffer.dir", "/tmp/spark_buffer")
    conf = conf.config("spark.hadoop.fs.s3a.fast.upload", "true")
    conf = conf.config("spark.hadoop.fs.s3a.fast.upload.buffer", "disk")
    conf = conf.config("spark.hadoop.fs.s3a.multipart.size", "5242880")  # 5MB
    conf = conf.config("spark.hadoop.fs.s3a.connection.maximum", "100")

    # Config HIVE
    conf = conf.config("spark.sql.catalogImplementation", "in-memory")

    session = conf.getOrCreate()

    # Enable detailed logging
    log4j = session._jvm.org.apache.log4j
    log4j.Logger.getLogger("org.apache.hadoop.fs.s3a").setLevel(log4j.Level.DEBUG)
    log4j.Logger.getLogger("org.apache.spark.internal.io.cloud").setLevel(
        log4j.Level.DEBUG
    )

    return session


def init_s3_filesystem(spark, bucket_name):
    """Initialize S3A filesystem for a bucket"""
    hadoop_conf = spark.sparkContext._jsc.hadoopConfiguration()
    s3_fs = spark._jvm.org.apache.hadoop.fs.s3a.S3AFileSystem()
    s3_fs.initialize(spark._jvm.java.net.URI("s3a://" + bucket_name), hadoop_conf)
    return s3_fs


def list_files_recursive(s3_fs, path, indent=""):
    """Recursively list files in a directory with pretty formatting"""
    try:
        files = s3_fs.listStatus(path)
        for file in files:
            if file.isDirectory():
                print(f"{indent}📁 {file.getPath().getName()}/")
                list_files_recursive(s3_fs, file.getPath(), indent + "  ")
            else:
                print(f"{indent}📄 {file.getPath().getName()} ({file.getLen()} bytes)")
    except Exception as e:
        print(f"{indent}❌ Error listing {path}: {str(e)}")


def read_json_file(spark, file_path):
    """Read a JSON file and display its contents"""
    try:
        print(f"\nReading {file_path}:")

        # First, verify the file exists and is accessible
        path_obj = spark._jvm.org.apache.hadoop.fs.Path(file_path)
        fs = path_obj.getFileSystem(spark.sparkContext._jsc.hadoopConfiguration())

        if not fs.exists(path_obj):
            print(f"❌ File does not exist: {file_path}")
            return None

        # Get file status for size info
        file_status = fs.getFileStatus(path_obj)
        file_size = file_status.getLen()
        print(f"File size: {file_size} bytes")

        # Try to read the file content
        print("1. Testing file accessibility...")
        input_stream = fs.open(path_obj)

        # Read first chunk to check for null bytes
        initial_chunk = bytearray(min(4096, file_size))
        bytes_read = input_stream.read(initial_chunk)

        # Count null bytes in the initial chunk
        null_byte_count = initial_chunk.count(b"\x00"[0])
        if null_byte_count > 0:
            print(
                f"⚠️ Warning: Found {null_byte_count} null bytes in the first {bytes_read} bytes"
            )
            print("This might indicate file corruption or incorrect transfer")

            # Try to find first non-null byte
            non_null_pos = -1
            for i, byte in enumerate(initial_chunk):
                if byte != 0:
                    non_null_pos = i
                    break

            if non_null_pos >= 0:
                print(f"First non-null byte found at position {non_null_pos}")
                print(f"Content starting from first non-null byte:")
                print(repr(initial_chunk[non_null_pos : non_null_pos + 100]))
            else:
                print("❌ No non-null bytes found in initial chunk")

        # Try reading more of the file
        print("\n2. Attempting to read more content...")
        try:
            # Seek back to start
            input_stream.seek(0)

            # Try reading larger chunks to find valid content
            chunk_size = 8192  # 8KB chunks
            content_chunks = []
            total_bytes = 0

            while total_bytes < min(file_size, 32768):  # Read up to 32KB
                chunk = bytearray(chunk_size)
                bytes_read = input_stream.read(chunk)
                if bytes_read <= 0:
                    break

                content_chunks.append(chunk[:bytes_read])
                total_bytes += bytes_read

                # Try to decode this chunk
                try:
                    decoded = chunk[:bytes_read].decode("utf-8")
                    if "{" in decoded or "[" in decoded:
                        print(
                            f"\nFound JSON-like content in chunk {len(content_chunks)}:"
                        )
                        start_idx = max(decoded.find("{"), decoded.find("["))
                        print(repr(decoded[start_idx : start_idx + 100]))
                except UnicodeDecodeError:
                    continue

            print(f"\nRead {total_bytes} bytes in {len(content_chunks)} chunks")

        except Exception as e:
            print(f"Error reading chunks: {str(e)}")
        finally:
            input_stream.close()

        print("\n3. Attempting to read with Spark DataFrame...")
        try:
            df = (
                spark.read.format("json")
                .option("mode", "PERMISSIVE")
                .option("columnNameOfCorruptRecord", "_corrupt_record")
                .option("multiLine", "true")
                .option("charset", "UTF-8")
                .option("inferSchema", "true")
                .option("prefersDecimal", "false")
                .option("allowNumericLeadingZeros", "true")
                .option("allowBackslashEscapingAnyCharacter", "true")
                .option("allowUnquotedFieldNames", "true")
                .option("allowSingleQuotes", "true")
                .option("allowComments", "true")
                .option("allowUnquotedControlChars", "true")
                .json(file_path)
            )

            print("✅ Successfully created DataFrame")
            print("\n4. Schema:")
            df.printSchema()

            print("\n5. Sample data:")
            df.show(5, truncate=False)

            return df

        except Exception as e:
            print(f"❌ Error creating DataFrame: {str(e)}")
            return None

    except Exception as e:
        print(f"❌ Error reading {file_path}")
        print(f"Error type: {type(e).__name__}")
        print(f"Error message: {str(e)}")
        import traceback

        print("Stack trace:")
        traceback.print_exc()
        return None


def list_directory_contents(spark, s3_fs, dir_path):
    """List contents of a specific directory and read JSON files"""
    print(f"\nListing contents of {dir_path}:")
    try:
        path_obj = spark._jvm.org.apache.hadoop.fs.Path(dir_path)
        if s3_fs.exists(path_obj):
            files = s3_fs.listStatus(path_obj)
            for file in files:
                path = file.getPath()
                if file.isDirectory():
                    print(f"📁 {path.getName()}/")
                else:
                    print(f"📄 {path.getName()} ({file.getLen()} bytes)")
                    if path.getName().endswith(".json"):
                        read_json_file(spark, str(path))
        else:
            print(f"❌ Path does not exist: {dir_path}")
    except Exception as e:
        print(f"❌ Error accessing path {dir_path}: {str(e)}")


def verify_s3_configuration(spark):
    """Verify and print S3 configuration"""
    print("\nVerifying S3 Configuration:")
    hadoop_conf = spark.sparkContext._jsc.hadoopConfiguration()
    config_items = [
        "fs.s3a.endpoint",
        "fs.s3a.impl",
        "fs.s3a.access.key",
        "fs.s3a.secret.key",
        "fs.s3a.path.style.access",
        "fs.s3a.aws.credentials.provider",
    ]
    for item in config_items:
        print(f"- {item}: {hadoop_conf.get(item)}")


def verify_file_integrity(spark, file_path, expected_size=None):
    """Verify file integrity and content"""
    try:
        print(f"\nVerifying file integrity for {file_path}")

        # Get file system and path object
        path_obj = spark._jvm.org.apache.hadoop.fs.Path(file_path)
        fs = path_obj.getFileSystem(spark.sparkContext._jsc.hadoopConfiguration())

        if not fs.exists(path_obj):
            print(f"❌ File does not exist: {file_path}")
            return False

        # Check file size
        file_status = fs.getFileStatus(path_obj)
        file_size = file_status.getLen()
        print(f"File size: {file_size} bytes")

        if expected_size and file_size != expected_size:
            print(f"❌ Size mismatch! Expected: {expected_size}, Actual: {file_size}")
            return False

        # Read file in chunks to verify content
        input_stream = fs.open(path_obj)
        try:
            chunk_size = 8192
            total_bytes = 0
            null_chunks = 0
            valid_chunks = 0

            while total_bytes < file_size:
                chunk = bytearray(min(chunk_size, file_size - total_bytes))
                bytes_read = input_stream.read(chunk)
                if bytes_read <= 0:
                    break

                # Check for null bytes
                null_count = chunk[:bytes_read].count(b"\x00"[0])
                if null_count == bytes_read:
                    null_chunks += 1
                else:
                    valid_chunks += 1

                total_bytes += bytes_read

                # Print progress every 1MB
                if total_bytes % (1024 * 1024) == 0:
                    print(f"Verified {total_bytes/1024/1024:.1f}MB...")

            print(f"\nVerification complete:")
            print(f"- Total bytes read: {total_bytes}")
            print(f"- Null chunks: {null_chunks}")
            print(f"- Valid chunks: {valid_chunks}")

            if null_chunks > 0 and valid_chunks == 0:
                print("❌ File appears to be corrupted (all null bytes)")
                return False

            return True

        finally:
            input_stream.close()

    except Exception as e:
        print(f"❌ Error verifying file: {str(e)}")
        return False


def test_s3_connection(spark, bucket_name):
    """Test S3 connection by writing and reading a test file"""
    print("\nTesting S3 connection with write/read test...")

    test_content = (
        '{"test": "data", "timestamp": "' + time.strftime("%Y-%m-%d %H:%M:%S") + '"}'
    )
    test_file = f"s3a://{bucket_name}/_test/test_file.json"

    try:
        # Write test file
        print(f"Writing test file to {test_file}")
        print(f"Test content ({len(test_content)} bytes): {repr(test_content)}")

        path_obj = spark._jvm.org.apache.hadoop.fs.Path(test_file)
        fs = path_obj.getFileSystem(spark.sparkContext._jsc.hadoopConfiguration())

        # Create parent directory if needed
        parent = path_obj.getParent()
        if not fs.exists(parent):
            fs.mkdirs(parent)

        # Try writing with DataFrame first
        print("\nAttempt 1: Writing with DataFrame...")
        try:
            # Create a small DataFrame with a single partition
            data = [{"test": "data", "timestamp": time.strftime("%Y-%m-%d %H:%M:%S")}]
            df = spark.createDataFrame(data)

            # Verify staging directories
            staging_dir = "/tmp/spark_staging"
            buffer_dir = "/tmp/spark_buffer"
            print(f"\nVerifying staging directories:")
            for d in [staging_dir, buffer_dir]:
                if not os.path.exists(d):
                    print(f"Creating directory: {d}")
                    os.makedirs(d, exist_ok=True)
                print(f"✅ Directory exists: {d}")

            print("\nWriting DataFrame with single partition...")
            (
                df.coalesce(1)  # Ensure single partition
                .write.mode("overwrite")
                .option("compression", "none")  # Disable compression for debugging
                .json(test_file)
            )

            # Verify the write operation
            print("\nVerifying write operation:")
            if fs.exists(path_obj):
                status = fs.getFileStatus(path_obj)
                print(f"✅ Directory created: {test_file}")
                print(f"- Modified: {status.getModificationTime()}")

                # List contents
                files = fs.listStatus(path_obj)
                print(f"\nContents of {test_file}/:")
                for f in files:
                    print(f"- {f.getPath().getName()} ({f.getLen()} bytes)")
            else:
                print(f"❌ Failed to create directory: {test_file}")

            print("✅ DataFrame write successful")

        except Exception as e:
            print(f"❌ DataFrame write failed: {str(e)}")
            print("\nChecking staging directories for issues:")
            for d in [staging_dir, buffer_dir]:
                if os.path.exists(d):
                    print(f"\nContents of {d}:")
                    for root, dirs, files in os.walk(d):
                        print(f"Directory: {root}")
                        for name in dirs:
                            print(f"  📁 {name}")
                        for name in files:
                            print(f"  📄 {name}")

            # Try direct file write as backup
            print("\nAttempt 2: Writing directly to file...")
            try:
                # Use buffered output stream
                output_stream = spark._jvm.java.io.BufferedOutputStream(
                    fs.create(path_obj),
                    8192,  # 8KB buffer
                )

                # Convert content to bytes and write
                content_bytes = test_content.encode("utf-8")
                print(f"Writing {len(content_bytes)} bytes...")
                output_stream.write(content_bytes)
                output_stream.flush()
                print("✅ Buffer flushed")
                output_stream.close()
                print("✅ Direct write successful")
            except Exception as e2:
                print(f"❌ Direct write failed: {str(e2)}")
                raise

        # Verify the written file exists
        if not fs.exists(path_obj):
            print("❌ File does not exist after write")
            return False

        # Get file info
        file_status = fs.getFileStatus(path_obj)
        print(f"\nFile status after write:")
        print(f"- Size: {file_status.getLen()} bytes")
        print(f"- Modified: {file_status.getModificationTime()}")
        print(f"- Owner: {file_status.getOwner()}")

        # Read back the content for verification
        print("\nReading back the content:")
        input_stream = fs.open(path_obj)
        try:
            content = input_stream.read().decode("utf-8")
            print(f"Read content ({len(content)} bytes): {repr(content)}")

            # Try parsing as JSON
            import json

            try:
                parsed = json.loads(content)
                print("✅ Content is valid JSON")
                print(f"Parsed content: {parsed}")
            except json.JSONDecodeError as je:
                print(f"❌ Content is not valid JSON: {str(je)}")

        finally:
            input_stream.close()

        # Clean up
        fs.delete(path_obj, True)  # Use recursive delete
        print("\n✅ Test file cleaned up")
        return True

    except Exception as e:
        print(f"❌ Error during S3 connection test: {str(e)}")
        import traceback

        traceback.print_exc()
        return False


def main(bucket_name):
    print(f"Starting S3 health check for bucket: {bucket_name}")
    start_time = time.time()

    # Initialize Spark and S3
    spark = create_spark_session()
    verify_s3_configuration(spark)

    # Test S3 connection first
    if not test_s3_connection(spark, bucket_name):
        print("❌ S3 connection test failed. Please check your S3 configuration.")
        spark.stop()
        return

    s3_fs = init_s3_filesystem(spark, bucket_name)

    # List all files recursively
    print("\nListing all files in bucket:")
    root_path = spark._jvm.org.apache.hadoop.fs.Path(f"s3a://{bucket_name}")
    list_files_recursive(s3_fs, root_path)

    # Check specific directory
    test_path = f"s3a://{bucket_name}/data/2025/01/06"
    list_directory_contents(spark, s3_fs, test_path)

    # Verify integrity of specific files
    for file_path in [f"s3a://{bucket_name}/data/2025/01/06/github_copilot_usage.json"]:
        verify_file_integrity(spark, file_path)

    end_time = time.time()
    duration = end_time - start_time

    print(f"\nHealth check completed in {duration:.2f} seconds")
    spark.stop()


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python healthcheck_spark_2.py <bucket_name>")
        sys.exit(1)

    bucket_name = sys.argv[1]
    main(bucket_name)
