import http from "k6/http";
import { sleep, check } from "k6";
import { Counter } from "k6/metrics";

export const requests = new Counter("http_reqs");

export const options = {
    stages: [
        { duration: "10s", target: 25 },        // ramp up users to 25 in 10 seconds
        { duration: "5s", target: 1000 },        // maintain 25 users for 10 seconds
        { duration: "30s", target: 1000 },        // maintain 25 users for 10 seconds
        { duration: "10s", target: 0 }          // ramp down to 0 users in 10 seconds
    ],
    thresholds: {
        // 'http_req_duration': ['p(90) < 500'],   // 90% of requests must finish within 500ms.
    },
};


export default function(data) {
    let get_all_photos = http.post("http://localhost:6969/api/v0/photos", JSON.stringify({}),  {});
    check(get_all_photos, {
        "get all photos summaries status": (r) => r.status === 200,
    });
};

