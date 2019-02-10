# Nebula OS alpine packages build crate
# We infer that there are /nebula and /nebula-work folders mounted
FROM alpine:latest

# Setup for creating packages
RUN apk add alpine-sdk
RUN adduser -G abuild -g "Alpine Package Builder" -s /bin/ash -D builder \
  && echo "builder ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
USER builder

# Set the working directory
WORKDIR /nebula-work

# Build the packages
CMD abuild-keygen -i -a ;\
    # Build nebula-baselayout
    cd /nebula/packages/nebula-baselayout ;\
    abuild checksum ;\
    abuild -r -P /nebula-work ;\
    # Build nebula-base
    cd /nebula/packages/nebula-base ;\
    abuild checksum ;\
    abuild -r -P /nebula-work ;\
    # Build nebula-init
    cd /nebula/packages/nebula-init ;\
    abuild checksum ;\
    abuild -r -P /nebula-work ;\
    # Re-own the directories
    sudo chown -R builder:abuild /nebula-work ;\
    cat /etc/apk/repositories ;\
    # Do some files maintenance
    cp /nebula-work/packages/. /nebula-work/ -r ;\
    rm /nebula-work/packages -rf