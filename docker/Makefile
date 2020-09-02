NAME=timereport-api
VERSION=0.0.1

need := 3.83.1
ok := $(filter $(need),$(firstword $(sort $(MAKE_VERSION) $(need))))



.DEFAULT_GOAL := start
.PHONY: start pull stop restart clean

pull:
	docker pull amazon/dynamodb-local

start: pull
	docker-compose up -d

stop:
	docker-compose stop

restart: stop start

clean: stop
	docker-compose rm -vf