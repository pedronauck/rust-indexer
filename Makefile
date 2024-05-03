start:
	docker compose up -d --build

stop:
	docker compose stop

clean:
	docker compose down --rmi local -v

logs:
	docker compose logs -f

reset:
	make clean && make start
