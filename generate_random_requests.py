import random
import string

def generate_random_http_requests(count):
    http_methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS", "HEAD"]

    def random_string(length):
        return ''.join(random.choices(string.ascii_lowercase + string.digits, k=length))

    requests = []

    for _ in range(count):
        method = random.choice(http_methods)
        path = f"/{random_string(5)}/{random_string(8)}"

        # Generate random headers
        num_headers = random.randint(1, 50)
        headers = {
            random_string(8): random_string(12) for _ in range(num_headers)
        }
        header_text = "".join(f"{key}: {value}\r\n" for key, value in headers.items())

        # Generate random body for methods that allow it
        body = ""
        if method in ["POST", "PUT", "PATCH"]:
            body_length = random.randint(100, 5000)
            body = ''.join(random.choices(string.ascii_letters + string.digits + " ", k=body_length))

        # Combine the request
        request_text = f"{method} {path} HTTP/1.1\r\n{header_text}\r\n{body}"
        requests.append(request_text)

    return requests

if __name__ == "__main__":
    x = int(input("Enter the number of random HTTP requests to generate: "))
    random_requests = generate_random_http_requests(x)
    with open("random_http_requests.txt", "w") as file:
        for i, request in enumerate(random_requests, 1):
            file.write(request)
            if i < x:
                file.write("\x00")
