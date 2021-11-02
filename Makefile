.SILENT:

### Containerized workflow (for Podman and Docker) ###

_IF_PODMAN := $(shell command -v podman 2> /dev/null)

define container-tool
	$(if $(_IF_PODMAN), podman, docker)
endef

IMAGE_TAG = kaptan-proxy
CONTAINER_NAME = kaptan-proxy

build-container:
ifeq ($(env), release)
	@echo Building production image
	$(call container-tool) build --build-arg RELEASE="TRUE" --no-cache -t $(IMAGE_TAG) -f .container/Containerfile .
else
	@echo Building development image
	$(call container-tool) build --no-cache -t $(IMAGE_TAG) -f .container/Containerfile .
endif

start-container:
	$(call container-tool) run --detach --name $(CONTAINER_NAME) --network host $(IMAGE_TAG)

stop-container:
	$(call container-tool) stop --time 1 $(CONTAINER_NAME)
	$(call container-tool) rm $(CONTAINER_NAME)

restart-container:
	$(MAKE) stop-container && $(MAKE) start-container

logs-container:
	$(call container-tool) logs --follow $(CONTAINER_NAME)

### Normal workflow ###

build: export JEMALLOC_SYS_WITH_MALLOC_CONF = background_thread:true,narenas:1,tcache:false,dirty_decay_ms:0,muzzy_decay_ms:0,metadata_thp:auto
build:
ifeq ($(env), release)
	@echo Release compilation has been started
	cargo build --release
else
	@echo Debug compilation has been started
	cargo build
endif

start: export JEMALLOC_SYS_WITH_MALLOC_CONF = background_thread:true,narenas:1,tcache:false,dirty_decay_ms:0,muzzy_decay_ms:0,metadata_thp:auto
start:
ifeq ($(env), release)
	@echo Release compilation has been started
	cargo run --release
else
	@echo Debug compilation has been started
	cargo run
endif

test:
	cargo test

install:
	@echo todo

clean:
	cargo clean

define ANNOUNCE_INIT_CFG:=
Creating 'cfg.toml' in /etc/kaptan-proxy

~ test
~ todo
~ bla

endef

init-cfg: export ANNOUNCE = $(ANNOUNCE_INIT_CFG)
init-cfg:
	echo "$$ANNOUNCE"

	mkdir -p /etc/kaptan-proxy
	cp .container/cfg.toml /etc/kaptan-proxy/cfg.toml

.PHONY: build-container start-contaienr stop-container \
	restart-container logs-container build start install \
	clean init-cfg
