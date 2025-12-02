#!/usr/bin/env python3
import asyncio
import websockets
import json
from datetime import datetime

# LLM generated, just to save time, binary message setup by me

async def handler(websocket, path):
    """Handle incoming WebSocket connections."""
    print(f"Client connected from {websocket.remote_address}")
    
    try:        
        while True:
            await asyncio.sleep(2)
            
            # Create binary message: "GOL" + 2-byte length + game of life universe
            header = b"GOL"
            width = 10
            height = 10
            data_size = width * height
            size_bytes = data_size.to_bytes(2, byteorder='big')  # 2-byte big-endian length
            payload = bytes(data_size)

            payload_l = [0] * (width*height)
            i = ((width * height) // 2) + width // 3
            # for j in range(0, width//2):
            payload_l[i - 1] = 1
            payload_l[i] = 1
            payload_l[i + 1] = 1

            i = ((width * height) // 2) + width-2
            # for j in range(0, width//2):
            payload_l[i - 1] = 1
            payload_l[i] = 1
            payload_l[i + 1] = 1
            
            payload = bytes(payload_l)
            binary_message = header + size_bytes + payload
            
            await websocket.send(binary_message)
            print(f"Sent binary message ({len(binary_message)} bytes total)")
            
    except websockets.exceptions.ConnectionClosed:
        print(f"Client disconnected from {websocket.remote_address}")
    except Exception as e:
        print(f"Error: {e}")

async def main():
    """Start the WebSocket server."""
    print("Starting WebSocket server on ws://localhost:8081")
    
    async with websockets.serve(handler, "localhost", 8081):
        print("Server is running. Press Ctrl+C to stop.")
        await asyncio.Future()

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\nServer stopped.")
