import http from "k6/http";
import { group } from "k6";
import { sleep, check } from "k6";
import { Counter } from "k6/metrics";

export const requests = new Counter("http_reqs");

export const options = {
  discardResponseBodies: true,
  scenarios: {
    health_check: {
      executor: "constant-vus",
      exec: "health_check",
      vus: 50,
      duration: "30s",
    },
    health_check_vus: {
      executor: "ramping-vus",
      exec: "health_check",
      stages: [
        { duration: "10s", target: 25 }, // ramp up users to 25 in 10 seconds
        { duration: "5s", target: 1000 }, // maintain 25 users for 10 seconds
        { duration: "30s", target: 1000 }, // maintain 25 users for 10 seconds
        { duration: "10s", target: 0 }, // ramp down to 0 users in 10 seconds
      ],
      startVUs: 50, // how large the initial pool of VUs would be
    },
    get_all_photos_vus: {
      executor: "ramping-vus",
      exec: "get_all_photos",
      stages: [
        { duration: "10s", target: 25 }, // ramp up users to 25 in 10 seconds
        { duration: "5s", target: 1000 }, // maintain 25 users for 10 seconds
        { duration: "30s", target: 1000 }, // maintain 25 users for 10 seconds
        { duration: "10s", target: 0 }, // ramp down to 0 users in 10 seconds
      ],
      startVUs: 50, // how large the initial pool of VUs would be
    },
    get_photo_vus: {
      executor: "ramping-vus",
      exec: "get_photo",
      stages: [
        { duration: "10s", target: 25 }, // ramp up users to 25 in 10 seconds
        { duration: "5s", target: 1000 }, // maintain 25 users for 10 seconds
        { duration: "30s", target: 1000 }, // maintain 25 users for 10 seconds
        { duration: "10s", target: 0 }, // ramp down to 0 users in 10 seconds
      ],
      startVUs: 50, // how large the initial pool of VUs would be
    },
  },
};

export function health_check(data) {
  let health = http.get("http://localhost:6969/health_check");
  check(health, {
    "health check status": (r) => r.status === 200,
  });
}
export function get_all_photos(data) {
  let get_all_photos = http.get("http://localhost:6969/api/v0/photos");
  check(get_all_photos, {
    "get all photos summaries status": (r) => r.status === 200,
  });
}

export function get_photo(data) {
  let get_photo = http.get("http://localhost:6969/api/v0/photos/1");
  check(get_photo, {
    "get photo status": (r) => r.status === 200,
  });
}
