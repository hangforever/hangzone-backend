clr_docker_volumes:
	docker volume rm $$(docker volume ls -q)