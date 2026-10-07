#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include <stdbool.h>

#include <vulkan/vulkan.h>
#include <GLFW/glfw3.h>

#define PANIC(ERROR, FORMAT, ...)                                                                                               \
	{                                                                                                                           \
		if (ERROR)                                                                                                              \
		{                                                                                                                       \
			fprintf(stderr, "%s -> %s -> %i -> Error(%i):\n\t" FORMAT, __FILE__, __FUNCTION__, __LINE__, ERROR, ##__VA_ARGS__); \
			raise(SIGABRT);                                                                                                     \
		}                                                                                                                       \
	}

void glfwErrorCallback(int error_code, const char *description)
{
	PANIC(error_code, "GLFW: %s", description)
}

void exitCallback(void)
{
	glfwTerminate();
}

typedef struct
{
	const char *window_title;
	int windows_width, windows_height;
	bool window_resizable;
	bool window_fullscreen;

	GLFWmonitor *window_monitor;
	GLFWwindow *window;

} State;

void setupErrorHandling()
{
	glfwSetErrorCallback(glfwErrorCallback);
	atexit(exitCallback);
}

void createWindow(State *state)
{
	glfwInit();
	glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
	glfwWindowHint(GLFW_RESIZABLE, state->window_resizable);

	if (state->window_fullscreen)
	{
		state->window_monitor = glfwGetPrimaryMonitor();
	}
	state->window = glfwCreateWindow(state->windows_width, state->windows_height, state->window_title, state->window_monitor, NULL);
}

void init(State *state)
{
	setupErrorHandling();
	createWindow(state);
}

void loop(State *state)
{
	while (!glfwWindowShouldClose(state->window))
	{
		glfwPollEvents();
	}
}

void cleanup(State *state)
{
	glfwDestroyWindow(state->window);
	state->window = NULL;
}

int main()
{
	State state = {
		.window_title = "Garage",
		.windows_width = 720,
		.windows_height = 480,
		.window_resizable = false,
		.window_fullscreen = false};

	init(&state);
	loop(&state);
	cleanup(&state);

	return 0;
}
