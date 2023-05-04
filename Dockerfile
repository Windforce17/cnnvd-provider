FROM ubuntu:22.04
COPY target/release/cnnvd-provider /app/cnnvd-provider

RUN 
RUN echo ' deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ jammy main restricted universe multiverse \n \
deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ jammy-updates main restricted universe multiverse \n \
deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ jammy-backports main restricted universe multiverse \n \
deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ jammy-security main restricted universe multiverse \n '> /etc/apt/sources.list && \
chmod +x /app/cnnvd-provider && \
apt-get update && \
apt-get install -y ca-certificates tzdata && \
ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime 
WORKDIR /app
CMD ["./cnnvd-provider"]