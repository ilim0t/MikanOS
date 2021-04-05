FROM ubuntu:20.04

RUN apt-get update && apt-get install -yq git ansible sudo
RUN git clone https://github.com/uchan-nos/mikanos-build.git /root/osbook
RUN cd /root/osbook/devenv && ansible-playbook -K -i ansible_inventory ansible_provision.yml

RUN apt-get install -y qemu dosfstools

# ENTRYPOINT [ "/root/osbook/devenv/run_qemu.sh" ]