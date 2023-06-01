bootstrap: install-dev-deps
	@pre-commit install

install-deps:
	@echo "[INFO] Installing dependencies"
	@cd backend && python -m pipenv install
	@cd frontend && yarn --production

install-dev-deps:
	@echo "[INFO] Install pre-requisite dependencies"
	@python -m pip install --no-input pipenv pre-commit
	@echo "[INFO] Installing dev dependencies"
	@cd backend && python -m pipenv install --dev
	@cd frontend && yarn

format-src:
	@echo "[INFO] Formatting backend source code using black"
	@cd backend && python -m pipenv run black src
	@cd backend && python -m pipenv run black tests
	@echo "[INFO] Formatting frontend source code using prettier"
	@cd frontend && yarn pretty

lint:
	@echo "[INFO] Linting backend source code using pylint"
	@cd backend && python -m pipenv run pylint --fail-under 7.5 src/service/*
	@echo "[INFO] Linting frontend source code using eslint"
	@cd frontend && yarn lint

bandit:
	@echo "[INFO] Linting source code using bandit to look for common security issues in python source"
	@cd backend && python -m pipenv run bandit -r src/serivce

type-check:
	@echo "[INFO] Checking static typing of source code using mypy"
	@cd backend && python -m mypy src/service --ignore-missing-imports

backend-test:
	@echo "[INFO] Running backend tests"
	@cd backend && export COVERAGE_FILE=./coverage/.cov && python -m pipenv run coverage run --source=src -m pytest -s --junitxml=target/results.xml tests/*.py
	@cd backend && export COVERAGE_FILE=./coverage/.cov && python -m pipenv run coverage html --directory=target/coverage --fail-under=10
	@cd backend && export COVERAGE_FILE=./coverage/.cov && python -m pipenv run coverage report
	@cd backend && export COVERAGE_FILE=./coverage/.cov && python -m pipenv run coverage xml -o target/coverage/coverage.xml

frontend-test:
	@echo "[INFO] Running frontend tests"
	@cd frontend && yarn test

deploy-backend:
	@echo "[INFO] Deploying backend"
	@cd backend && pipenv requirements > requirements.txt
	@cd backend && npx sls deploy -s prd

build-frontend:
	@echo "[INFO] Deploying frontend"
	@cd frontend && yarn build
	