FROM nginx:latest

COPY nginx.conf /etc/nginx/conf.d/default.conf

RUN mkdir -p /var/www/html
COPY big.json /var/www/html/big.json
COPY big-data.json /var/www/html/big-data.json
COPY medium.json /var/www/html/medium.json
COPY medium-data.json /var/www/html/medium-data.json
COPY employees.json /var/www/html/employees.json
COPY employees-data.json /var/www/html/employees-data.json

CMD ["nginx", "-g", "daemon off;"]
