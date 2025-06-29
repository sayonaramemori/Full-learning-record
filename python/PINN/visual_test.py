import numpy as np
import matplotlib.pyplot as plt
import os

# Function to save the visualization as an image
def save_image(x, y, u, v, p, time, filename="flow_output"):
    # Create a figure
    plt.figure(figsize=(6, 5))

    # Plot velocity field magnitude
    velocity_magnitude = np.sqrt(u**2 + v**2)  # Magnitude of the velocity vector
    plt.contourf(x, y, velocity_magnitude.reshape(len(y), len(x)), cmap='coolwarm')
    plt.colorbar(label="Velocity Magnitude")
    plt.title(f"Velocity Field at t={time:.1f}s")
    plt.xlabel("X")
    plt.ylabel("Y")
    
    # Save the image
    output_filename = f"./pics/{filename}_t{time:.1f}.png"
    plt.savefig(output_filename, dpi=300)
    plt.close()  # Close the plot to free memory

def save_stream_line(x, y, u, v, p, time, filename="flow_stream"):
    plt.figure()
    plt.streamplot(x, y, u, v, density=1.5)
    plt.title("DeepXDE + PyTorch (GPU) with L-BFGS: Lid-Driven Cavity (Re=100)")
    plt.xlabel("x")
    plt.ylabel("y")
    # plt.show()
    output_filename = f"./pics/{filename}_t{time:.1f}.png"
    plt.savefig(output_filename, dpi=300)
    plt.close()  # Close the plot to free memory


if __name__ == '__main__':
    x = np.linspace(0, 1, 100)  # X coordinates (adjust to your domain)
    y = np.linspace(0, 1, 100)  # Y coordinates (adjust to your domain)
    X, Y = np.meshgrid(x, y)  # Create a meshgrid for the 2D domain
    x_flat = X.flatten()
    y_flat = Y.flatten()
    for t in np.arange(0.0, 1.1, 0.1):  # Time steps 0.0 to 1.0 with step size 0.1
        # Replace these with your actual model predictions
        u = np.sin(np.pi * x_flat) * np.cos(np.pi * y_flat) * np.cos(np.pi * t)  # Simulated u-velocity
        v = -np.cos(np.pi * x_flat) * np.sin(np.pi * y_flat) * np.cos(np.pi * t)  # Simulated v-velocity
        p = np.zeros_like(x_flat)  # Simulated pressure (replace with actual pressure field)

        # Save the results as an image for the current time step
        save_image(x, y, u, v, p, t, filename="flow_output")

