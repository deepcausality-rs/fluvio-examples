
# QD communication protocol

The communication between the QD client and QD gateway follows a simple protocol.

1)	The client sends a login message with its client ID to the gateway.
   * If the client is already logged in, a client error message gets returned. 
   * If the client is not yet known, the login process starts. Notice, the gateway only returns error messages, but no login success message and that means it is the application’s responsibility to monitor the QD client for errors. If there is no error, it is safe to proceed.

2) Once connected, the client can send request either trade data or sampled OHLCV data at a resolution defined in the request message.
      * The gateway returns an error if the requested data are unavailable.
      * If the data are available, the gateway starts the data streaming.

   
3)	When no further data are needed, the QD client is supposed to send a logout message to the gateway. If the client does not send a logout, the next login attempt with the same client ID will result in an error.  


## Important details

* The QD client upon connection sends the login message automatically. 
* When the QD client has been created, the application can immediately request data. 
* The application logs out simply by calling the close method of the QD client, which sends the logout message. 
