.PHONY: deploy
deploy:
	docker pull ghcr.io/dupreehkuda/clubvent-prod:latest
	docker run -d --rm --network="host" --env-file=".env" ghcr.io/dupreehkuda/clubvent-prod:latest