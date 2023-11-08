FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld libaio1 \
   && apt-get autoremove -y && apt-get clean -y \
   && mkdir -p /opt/oracle \
   && wget -O /opt/oracle/oracle2111000.zip https://download.oracle.com/otn_software/linux/instantclient/2111000/instantclient-basic-linux.x64-21.11.0.0.0dbru.zip \
   && unzip /opt/oracle/oracle2111000.zip -d /opt/oracle/ \
   && echo /opt/oracle/instantclient_21_11 | sudo tee /etc/ld.so.conf.d/oracle.conf \
   && ldconfig
