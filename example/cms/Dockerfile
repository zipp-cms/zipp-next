# The cms of a site as an image: one release binary with the studio and
# the migrations, data in /data. See docs/operations.md, Hosting.
#
# For the example the context is the repository (zipp is a path
# dependency): docker build -f example/cms/Dockerfile -t example-cms .
# A site's own cms crate with zipp from crates.io builds from the
# site's repository; change PACKAGE to its name.

FROM rust:1-trixie AS build
ARG PACKAGE=example-cms
WORKDIR /src
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/src/target \
	cargo build --release --locked -p $PACKAGE \
	&& cp target/release/$PACKAGE /usr/local/bin/cms

FROM debian:trixie-slim
# the AI chat and SMTP talk TLS to other servers
RUN apt-get update \
	&& apt-get install -y --no-install-recommends ca-certificates \
	&& rm -rf /var/lib/apt/lists/*
RUN useradd --uid 1000 --create-home zipp \
	&& mkdir /data && chown zipp:zipp /data
COPY --from=build /usr/local/bin/cms /usr/local/bin/cms

USER zipp
WORKDIR /data
VOLUME /data
# the database, the uploads and a backup a day in the volume; set
# ZIPP_GRAPHQL_TOKEN, ZIPP_PUBLIC_URL, ZIPP_SITE_URL and the mail ones
# when running it
ENV ZIPP_LISTEN=0.0.0.0:8090 \
	ZIPP_DB=/data/zipp.db \
	ZIPP_FILES=/data/files \
	ZIPP_BACKUP_DIR=/data/backups
EXPOSE 8090
ENTRYPOINT ["cms"]
CMD ["serve"]
