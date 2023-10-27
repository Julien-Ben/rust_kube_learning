include .env
BUILTIMAGE := rust-server-image
PUSHEDIMAGE := julienben/rust-learning:latest

verify-variables:
	@if [[ -z "${MONGOURI}" ]]; then \
    	echo "ERROR: Your .env file is not correctly defined"; \
    	exit 1; \
      fi; \
      if [[ -z "${DOCKER_PASSWORD}" ]]; then \
		echo "ERROR: Your .env file is not correctly defined"; \
		exit 1; \
      fi

#============== DEPLOYMENT ==============
hub-login: verify-variables
	docker login -u "${DOCKER_USERNAME}" -p "${DOCKER_PASSWORD}" docker.io

deploy-kubernetes: verify-variables
	kubectl create secret generic mongouri-secret --namespace rust-server --from-env-file=.env
 #TODO complete

build-container:
	docker build --tag $(BUILTIMAGE) .

run-container:
	docker run -p 8080:8080 $(PUSHEDIMAGE)

push-container: hub-login
	docker tag $(BUILTIMAGE) $(PUSHEDIMAGE)
	docker push $(PUSHEDIMAGE)

docker-deploy: build-container push-container

#============== PREREQUISITES ==============
# Prerequisites rull haven't been tested yet
# Work only for Mac/Linux

install-kind:
#TODO complete

create-cluster:
#TODO complete

install-rust:
	curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

prerequisites: install-kind install-rust
#TODO complete
