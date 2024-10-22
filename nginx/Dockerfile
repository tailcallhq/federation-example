FROM nginx:latest

COPY nginx.conf /etc/nginx/conf.d/default.conf

RUN mkdir -p /var/www/html
COPY big.json /var/www/html/big.json
COPY big-data.json /var/www/html/big-data.json

EXPOSE 8090 4006

CMD ["nginx", "-g", "daemon off;"]
