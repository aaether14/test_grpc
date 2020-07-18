import grpc

import proto.say_pb2_grpc
from proto.say_pb2 import SayRequest

def run():
    with grpc.insecure_channel('localhost:50051') as channel:
        stub = proto.say_pb2_grpc.SayStub(channel)
        response = stub.Send(SayRequest(name='this test', 
            children=[SayRequest(name='lol'), SayRequest(name='123', children=[
                SayRequest(name='last_one')
            ])]))
        print("Greeter client received: " + response.message)

run()