#!/bin/bash
# Script to set up Redis for GitHub collector v2

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}GitHub Collector Redis Setup${NC}"
echo -e "This script will help you set up Redis for the GitHub collector v2"
echo

# Check if Redis is already installed
if command -v redis-server &> /dev/null; then
    echo -e "${GREEN}Redis is already installed!${NC}"
    redis-server --version
else
    echo -e "${YELLOW}Redis is not installed. Installing...${NC}"
    
    # Detect OS
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        if command -v apt-get &> /dev/null; then
            # Debian/Ubuntu
            sudo apt-get update
            sudo apt-get install -y redis-server
        elif command -v yum &> /dev/null; then
            # CentOS/RHEL
            sudo yum install -y redis
        elif command -v dnf &> /dev/null; then
            # Fedora
            sudo dnf install -y redis
        else
            echo -e "${RED}Unsupported Linux distribution. Please install Redis manually.${NC}"
            exit 1
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        if command -v brew &> /dev/null; then
            brew install redis
        else
            echo -e "${RED}Homebrew not found. Please install Homebrew first:${NC}"
            echo "ruby -e \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)\""
            exit 1
        fi
    else
        echo -e "${RED}Unsupported operating system. Please install Redis manually.${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}Redis installed successfully!${NC}"
fi

# Check if Redis is running
if redis-cli ping &> /dev/null; then
    echo -e "${GREEN}Redis server is already running!${NC}"
else
    echo -e "${YELLOW}Starting Redis server...${NC}"
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        sudo systemctl start redis || sudo service redis-server start
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        brew services start redis
    fi
    
    # Wait for Redis to start
    sleep 2
    
    # Check again
    if redis-cli ping &> /dev/null; then
        echo -e "${GREEN}Redis server started successfully!${NC}"
    else
        echo -e "${RED}Failed to start Redis server. Please start it manually.${NC}"
        exit 1
    fi
fi

# Create a test database for GitHub collector
echo -e "\n${BLUE}Setting up a test database for GitHub collector...${NC}"

# Flush the selected database (default: 0) to ensure it's clean
redis-cli flushdb
echo -e "${GREEN}Test database created and flushed!${NC}"

# Create environment variables script
echo -e "\n${BLUE}Creating environment variables script...${NC}"
cat > redis_env.sh << 'EOL'
#!/bin/bash
# Environment variables for GitHub collector v2 with Redis

# GitHub API token
export GITHUB_TOKEN=""  # <-- Add your GitHub token here

# Redis configuration
export USE_REDIS=true
export REDIS_HOST=localhost
export REDIS_PORT=6379
export REDIS_DB=0
export REDIS_PASSWORD=""  # Set only if you configured a password

# Log level
export LOG_LEVEL=INFO

# Print current configuration
echo "GitHub Collector v2 Redis Configuration:"
echo "------------------------------------------"
echo "USE_REDIS:       $USE_REDIS"
echo "REDIS_HOST:      $REDIS_HOST"
echo "REDIS_PORT:      $REDIS_PORT"
echo "REDIS_DB:        $REDIS_DB"
echo "REDIS_PASSWORD:  ${REDIS_PASSWORD:-(not set)}"
echo "LOG_LEVEL:       $LOG_LEVEL"
echo "------------------------------------------"
echo "GITHUB_TOKEN:    ${GITHUB_TOKEN:0:4}$([ -n "$GITHUB_TOKEN" ] && echo "...")"
echo "------------------------------------------"
echo
echo "To use these settings in your current shell session:"
echo "source redis_env.sh"
EOL

chmod +x redis_env.sh
echo -e "${GREEN}Environment variables script created: redis_env.sh${NC}"
echo -e "${YELLOW}Don't forget to add your GitHub token to the script!${NC}"

# Test Redis connection
echo -e "\n${BLUE}Testing Redis connection...${NC}"
if ping_result=$(redis-cli ping); then
    echo -e "${GREEN}Redis connection successful: $ping_result${NC}"
    echo -e "${GREEN}Redis info:${NC}"
    redis-cli info server | grep redis_version
    redis-cli info server | grep os
    redis-cli info memory | grep used_memory_human
else
    echo -e "${RED}Failed to connect to Redis server. Please check your configuration.${NC}"
    exit 1
fi

echo -e "\n${BLUE}Redis Setup Complete!${NC}"
echo -e "${GREEN}To get started:${NC}"
echo -e "1. Edit redis_env.sh to add your GitHub token"
echo -e "2. Run: source redis_env.sh"
echo -e "3. Run the example: python -m src.github_collector_v2.examples.redis_example"
echo

# Final tips
echo -e "${YELLOW}Useful Redis commands:${NC}"
echo -e "- redis-cli ping                # Test connection"
echo -e "- redis-cli info keyspace       # Show database statistics"
echo -e "- redis-cli keys \"*\"            # List all keys"
echo -e "- redis-cli get KEY             # Get value for a key"
echo -e "- redis-cli flushdb             # Clear current database"
echo -e "- redis-cli monitor             # Monitor Redis commands in real-time" 