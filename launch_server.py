import http.server

class CustomHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def list_directory(self, path):
        # Check if the directory being listed is the one you want to hide
        if path.endswith('pkg/'):
            # Return a forbidden message (403) instead of the directory listing
            self.send_error(403, "Access denied")
            return None

        # For other directories, use the default behavior
        return super().list_directory(path)

# Start the server with the custom request handler
http.server.test(HandlerClass=CustomHTTPRequestHandler)