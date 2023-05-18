build:
	export DOCKER_BUILDKIT=1 && docker build --network host -t http-pubsub-publisher .

up: build
	docker run -it --name http-pubsub-publisher --rm --network host -p "8080:8080" http-pubsub-publisher

clean:
	docker rm -f http-pubsub-publisher
	docker rmi http-pubsub-publisher
