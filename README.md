## rust_reference_codes

# multi_thread_example
Using RUST mpsc channels for threads
(multi-producer-single-consumer and multi-producer-multi-consumer)

# Daemon_CLI
A simple APP that supports:
1) a background daemon process running, waiting for requests from clients
2) a CLI with which user can invoke a request
3) a cli process that sends the request to the daemon and waits for a response
4) The Daemon spawns a separate thread for every incoming request, processes it and sends the response back to the calling CLI
5) the cli process receives the response, prints some output and exits
6) Daemon continues to live, waiting for any new requests

This APP can be extended to different uses in the systems side programming world!

# ai_networking
Make a simple Flow-control-unit processing example, where based on credits, messages flow between sender and receiver

