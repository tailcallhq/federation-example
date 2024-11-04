FROM node:23-alpine3.19
WORKDIR /usr/src/app
COPY main.graphql main.graphql
COPY upstream.graphql upstream.graphql
COPY employees/employees.graphql employees/employees.graphql
COPY family/family.graphql family/family.graphql
COPY hobbies/hobbies.graphql hobbies/hobbies.graphql
COPY mood/mood.graphql mood/mood.graphql
COPY products/products.graphql products/products.graphql
RUN npm i -g @tailcallhq/tailcall
EXPOSE 8030
CMD ["tailcall", "start", "main.graphql"]