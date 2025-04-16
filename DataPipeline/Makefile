
# Makefile for managing DB migrations using dbmate

DBMATE=dbmate
ENV_FILE=.env

## TSDB (TimescaleDB) ##
migrate:tsdb:
	@echo "Running TimescaleDB migrations..."
	$(DBMATE) --env-file $(ENV_FILE) --env DATABASE_URL_TSDB --migrations-dir ./migrations/tsdb up

rollback:tsdb:
	@echo "Rolling back TimescaleDB migrations..."
	$(DBMATE) --env-file $(ENV_FILE) --env DATABASE_URL_TSDB --migrations-dir ./migrations/tsdb down

## REDSHIFT ##
migrate:redshift:
	@echo "Running Redshift migrations..."
	$(DBMATE) --env-file $(ENV_FILE) --env DATABASE_URL_REDSHIFT --migrations-dir ./migrations/redshift up

rollback:redshift:
	@echo "Rolling back Redshift migrations..."
	$(DBMATE) --env-file $(ENV_FILE) --env DATABASE_URL_REDSHIFT --migrations-dir ./migrations/redshift down
