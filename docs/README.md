# Signal Neural Network (SNN)

As in the readme, this is based around an idea I have, no idea if it already exists. but I did found the neural coding wiki article.

The idea is that our neural networks talk with signals, not numbers.
We can thus convert peaks into 1 and troughs into 0

Whereby each digit in a number is also based on the timing of the signal.

**Example**:

So lets take a simple signal:

```
11100100
```

This is a signal that has 3 peaks, then 2 troughs, then 1 peak, then stops. Our neural network will then transform this signal into a number, which represents the respondse of the network to this signal.

- Note: is the transformation of the signal is 0 then you could see it as if the signal is blocked by the neuron.

## Asynchronous

Our brains don't work synchronously, and I assume here, that signals comining in later or earlier maybe doesn't matter as long as it arrives with a certain time frame.
For this reason, I assume that we can process all inputs at the same time.

## Neural Nodes / Neural Coding

## Transformers

One of the nodes in the network will be a mapped transformation. Which can do some interesting things with the signal. That are either acting like a filter, if statement, switch statement, or even a mathematical operation.

See more about this in the [docs](./neural_coding.md#mapped-transformation).

## Combinators

The combinators are the nodes that combine the signals from multiple inputs into a single output. This can be done by summing the signals, averaging them, or applying more complex operations like logical AND/OR/XOR.

With transformers you can do some great things, of node identification, like if a signal is a letter or a number, or even more complex operations like mathematical operations. Combinators can them combine them to keep certain bits, or record activity of the network. etc. etc.

