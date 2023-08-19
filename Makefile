.PHONY: deploy
deploy:
	docker pull ghcr.io/dupreehkuda/clubvent-prod:latest
	docker kill clubvent
	docker rm clubvent
	docker run -d --restart=always --network="host" --env-file=".env" --name clubvent ghcr.io/dupreehkuda/clubvent-prod:latest