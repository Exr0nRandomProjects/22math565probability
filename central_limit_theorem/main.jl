using Plots
using Distributions;
using Statistics;
using ProgressMeter;

num_samples = 30

function sample_N(n, sample)
    ret = zeros(n)
    ret = map(x -> sample(), ret)

    mean = sum(ret) / n

    return mean
end

# function get_stddev(arr)
#     mean = sum(arr) / length(arr)
#     return mean, stdm(arr)
# end

println("starting computation...")

function plot_central_limit_thm(n_rounds, n_samples, sample_fn)
    means = [sample_N(n_samples, sample_fn) for _ = 1:n_rounds]
    println("means len: $(length(means)), stddev: $(std(means))")
    for p in 2:6
        p = histogram(means[1:10^p], bins=40, title="rounds: $(10^p)")
        display(p)
        println("press enter to continue")
        readline()
    end
end

plot_central_limit_thm(1e6, num_samples, rand)
