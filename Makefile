up:
	docker compose up -d
down:
	docker compose down
db: #パスワードはpassword
	docker compose exec postgres psql -h postgres -U user -d develop
seed:
	docker compose exec postgres psql -h postgres -U user -d develop -f seeding/init.sql

