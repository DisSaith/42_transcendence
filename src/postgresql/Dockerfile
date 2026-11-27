FROM postgres:16
COPY conf/init.sql /docker-entrypoint-initdb.d/init.sql
HEALTHCHECK --interval=5s --timeout=3s --start-period=10s --retries=10 \
CMD pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB" || exit 1
