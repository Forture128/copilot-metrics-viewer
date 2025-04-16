airflow connections add 'redis_default' \
    --conn-type 'redis' \
    --conn-host 'data-pipeline-redis' \
    --conn-port 6379 \
    --conn-extra '{"db": 0}'