import http from "k6/http";
import { check } from "k6";

export const options = {
  stages: [{ duration: "30s", target: 50 }],
};

export default function () {
  let res = http.get("http://127.0.0.1:8080/employees");
  check(res, {
    "is status 200": (r) => r.status === 200,
  });
}
