FROM gitea/gitea:latest

RUN mkdir -p /data/gitea/conf
COPY gitea.db /data/gitea/gitea.db
COPY app.ini /data/gitea/conf/app.ini

EXPOSE 3000
EXPOSE 22
