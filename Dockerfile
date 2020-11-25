FROM nicklehmann/poetry:py3.8-latest-alpine

RUN apk add build-base libffi-dev openssl-dev

ADD poetry.lock pyproject.toml ./

RUN poetry install $(test "$YOUR_ENV" == production && echo "--no-dev") --no-interaction --no-ansi

COPY . .

CMD ["python", "mangi/bot.py"]
