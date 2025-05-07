//Data strucutres and loopings rsr 😎

use std::collections::HashMap;

#[derive(Debug), PartialEq, Eq, Hash, Clone)]
struct Node{
    id: String,
    ip_address: String,
    port:u32,
    status: StatusNode
}
enum StatusNode{
    online,
    offline,
    busy,
    unknown,
}

fn main(){
    println!("🚀 Bem-vindo ao Playground de Estruturas de Dados e Loops 🚀\n");

    println!("--- 1 Arrays: our ports ---\n");
    let pattern_ports = [9393, 666, 999, 333];

    println!("Pattern Ports: {:?}", pattern_ports); 

    // +++ VECTORS (Vec<T>): A Lista Flexível +++
    println!("\n--- 2 Vectors: our ports ---\n");
    let mut ports = vec![9393, 666, 999, 333];

    // Adding nodes 
    node_of_network(Node{
        id: String::from("alphaNode"),
        ip_address: String::from("192.168.0.1"),
        port: 9393,
        status: StatusNode::online,
    }
}