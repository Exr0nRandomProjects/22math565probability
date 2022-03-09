using Plots
using Distributions;

num_samples = 30

function sample_N(n, sample)
    ret = 0
    for i in 1:n
        ret += sample()
    end

    return ret/n
end

println("starting computation...")

function plot_central_limit_thm(n_rounds, n_samples, sample_fn)
    p = histogram(randn(1000), bins=:scott, weights=repeat(1:5, outer=200))
    display(p)
    println("press enter to continue")
    readline()
end

plot_central_limit_thm(10000, num_samples, rand)
