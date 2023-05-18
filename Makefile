build:
	export DOCKER_BUILDKIT=1 && docker build --network host -t http-async-publisher .

up: build
	docker run -it --name http-async-publisher --rm --network host -p "8080:8080" http-async-publisher

clean:
	docker rm -f http-async-publisher
	docker rmi http-async-publisher
