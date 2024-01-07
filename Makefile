docker-build:
	docker buildx build . --tag request-reflector

docker-run: docker-build
	docker run -it --rm --name request-reflector request-reflector:latest