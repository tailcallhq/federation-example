import http from 'k6/http';
import { check } from 'k6';

export const options = {
  stages: [
    { duration: '30s', target: 50 },
    //{ duration: '1m30s', target: 100 },
    //{ duration: '20s', target: 0 },
  ],
};

export default function () {

  let res = http.post('http://localhost:8080/employees');
  check(res, {
    'is status 200': (r) => r.status === 200,
  });
}