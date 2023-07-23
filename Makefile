.PHONY: deploy
deploy:
	docker pull ghcr.io/dupreehkuda/clubvent-prod:latest
	docker run --rm --network="host" --env-file=".env" clubvent