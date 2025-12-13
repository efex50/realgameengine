import http.server
import socketserver

import sys

print(sys.argv)
global PORT
try:
    PORT = int(sys.argv[1])
except:
    PORT = 8080

print(PORT)

Handler = http.server.SimpleHTTPRequestHandler

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print(f"Serving at http://localhost:{PORT}")
    httpd.serve_forever()
