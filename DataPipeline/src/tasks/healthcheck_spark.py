from pyspark.sql import SparkSession


def main():
    """Simple Spark job to test connectivity"""
    spark = SparkSession.builder.appName("HealthCheck").getOrCreate()

    # Create a simple test DataFrame
    test_data = [("test",)]
    df = spark.createDataFrame(test_data, ["col1"])

    # Perform a simple action
    count = df.count()
    print(f"Test DataFrame count: {count}")

    spark.stop()
    return count


if __name__ == "__main__":
    main()
