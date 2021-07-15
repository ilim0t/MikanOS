FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -yq git ansible && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
RUN git clone https://github.com/uchan-nos/mikanos-build.git /root/osbook
RUN cd /root/osbook/devenv && ansible-playbook -K -i ansible_inventory ansible_provision.yml