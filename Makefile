up:
	docker compose up -d
down:
	docker compose down
db: #パスワードはpassword
	docker compose exec postgres psql -h postgres -U user -d develop
initial-migrate:
	docker compose exec postgres psql -h postgres -U user -d develop -f ./migrations/create-users-table.sql
