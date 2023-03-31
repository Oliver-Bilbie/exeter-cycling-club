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

unit-test:
	@echo "[INFO] Running unit tests"
	@cd backend && export COVERAGE_FILE=./target/coverage/.cov_unit && python -m pipenv run coverage run --source=src/service -m pytest -s --junitxml=target/unit-test/unit-result.xml src/test/unit/*.py
	@cd backend && export COVERAGE_FILE=./target/coverage/.cov_unit && python -m pipenv run coverage html --directory=target/unit-test/coverage --fail-under=60
	@cd backend && export COVERAGE_FILE=./target/coverage/.cov_unit && python -m pipenv run coverage report
	@cd backend && export COVERAGE_FILE=./target/coverage/.cov_unit && python -m pipenv run coverage xml -o target/unit-test/coverage/coverage.xml

int-test:
	@echo "[INFO] Running component integration tests"
	@cd backend && export COVERAGE_FILE=./target/coverage/.cov_int && python -m pipenv run coverage run --source=src/service -m pytest -s --junitxml=target/integration-test/integration-result.xml src/test/integration/*.py
	@cd backend && export COVERAGE_FILE=./target/coverage/.cov_int && python -m pipenv run coverage html --directory=target/integration-test/coverage --fail-under=50
	@cd backend && export COVERAGE_FILE=./target/coverage/.cov_int && python -m pipenv run coverage report
	@cd backend && export COVERAGE_FILE=./target/coverage/.cov_int && python -m pipenv run coverage xml -o target/integration-test/coverage/coverage.xml

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
	