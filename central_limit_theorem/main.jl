using Plots
using Distributions;
using Statistics;
using ProgressMeter;

num_samples = 50
pow_rounds = 5

function sample_N(n, dist)
    ret = zeros(n)
    ret = map(x -> rand(dist), ret)

    mean = sum(ret) / n

    return mean
end

println("starting computation... (num_samples = $(num_samples))")

function plot_central_limit_thm(n_rounds, n_samples, dist)
    means = [sample_N(n_samples, dist) for _ = 1:n_rounds]
    println("means len: $(length(means)), stddev: $(std(means))")
    for p in 2:log(10^pow_rounds)
        plot = histogram(means[1:Int(floor(exp(p)))], bins=40, title="rounds: $(Int(floor(exp(p))))")
        display(plot)
        println("press enter to continue (std = $(std(means[1:Int(floor(exp(p)))])))")
        readline()
    end
end

# plot_central_limit_thm(10^pow_rounds, num_samples, Normal(0, 1))
plot_central_limit_thm(10^pow_rounds, num_samples, Exponential())
# plot_central_limit_thm(10^pow_rounds, num_samples, Chisq(3))

# how large is sufficiently large?
# Chisq: n = 10 -> 2-8k, 20 -> 4-8k, 50 -> 1-2k, 100: 400-1k, 1k -> 200-400
# Exp: 10 -> 1k-3k, 20 -> 400 - 1k, 50 -> 200 - 400
