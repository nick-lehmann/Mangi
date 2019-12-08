FROM nicklehmann/poetry:3.8-alpine3.10

RUN apk add build-base libffi-dev openssl-dev

ADD poetry.lock pyproject.toml ./

RUN poetry install $(test "$YOUR_ENV" == production && echo "--no-dev") --no-interaction --no-ansi

COPY . .

CMD ["python", "mangi/bot.py"]
