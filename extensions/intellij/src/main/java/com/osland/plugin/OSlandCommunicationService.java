package com.osland.plugin;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.intellij.openapi.components.ApplicationComponent;
import com.intellij.openapi.components.ServiceManager;
import com.intellij.openapi.diagnostic.Logger;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.io.OutputStreamWriter;
import java.io.PrintWriter;
import java.util.Map;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class OSlandCommunicationService implements ApplicationComponent {
    private static final Logger logger = Logger.getInstance(OSlandCommunicationService.class);
    private static final String OSLAND_EXECUTABLE = "osland";
    private static final ObjectMapper objectMapper = new ObjectMapper()
            .enable(SerializationFeature.INDENT_OUTPUT);
    private static OSlandCommunicationService instance;

    private Process oslandProcess;
    private PrintWriter outputWriter;
    private ExecutorService executorService;
    private MessageListener messageListener;

    public interface MessageListener {
        void onMessage(String message);
        void onError(String error);
    }

    @Override
    public void initComponent() {
        logger.info("Initializing OSland Communication Service");
        executorService = Executors.newSingleThreadExecutor();
        instance = this;
    }

    @Override
    public void disposeComponent() {
        logger.info("Disposing OSland Communication Service");
        stopOSland();
        if (executorService != null) {
            executorService.shutdown();
        }
    }

    public boolean startOSland() {
        try {
            // Start OSland process
            ProcessBuilder processBuilder = new ProcessBuilder(OSLAND_EXECUTABLE, "--headless");
            processBuilder.redirectErrorStream(true);
            oslandProcess = processBuilder.start();

            // Get input and output streams
            outputWriter = new PrintWriter(oslandProcess.getOutputStream(), true);
            InputStream inputStream = oslandProcess.getInputStream();

            // Start reading from OSland
            executorService.submit(() -> {
                try (BufferedReader reader = new BufferedReader(new InputStreamReader(inputStream))) {
                    String line;
                    while ((line = reader.readLine()) != null) {
                        if (messageListener != null) {
                            messageListener.onMessage(line);
                        }
                    }
                } catch (IOException e) {
                    logger.error("Error reading from OSland process", e);
                    if (messageListener != null) {
                        messageListener.onError(e.getMessage());
                    }
                }
            });

            logger.info("OSland process started successfully");
            return true;
        } catch (IOException e) {
            logger.error("Failed to start OSland process", e);
            if (messageListener != null) {
                messageListener.onError(e.getMessage());
            }
            return false;
        }
    }

    public void stopOSland() {
        if (oslandProcess != null) {
            oslandProcess.destroy();
            try {
                oslandProcess.waitFor();
            } catch (InterruptedException e) {
                logger.error("Error stopping OSland process", e);
                Thread.currentThread().interrupt();
            }
            oslandProcess = null;
            outputWriter = null;
            logger.info("OSland process stopped");
        }
    }

    public void sendCommand(String command, Object data) {
        if (outputWriter == null) {
            logger.error("OSland process is not running");
            return;
        }

        try {
            Object commandData;
            
            // Handle ComponentNode specially since it's an inner class
            if (data instanceof OSlandToolWindow.ComponentNode) {
                OSlandToolWindow.ComponentNode componentNode = (OSlandToolWindow.ComponentNode) data;
                commandData = Map.of(
                        "id", componentNode.getId(),
                        "name", componentNode.getName(),
                        "type", componentNode.getType()
                );
            } else {
                commandData = data;
            }
            
            CommandMessage message = new CommandMessage(command, commandData);
            String json = objectMapper.writeValueAsString(message);
            outputWriter.println(json);
            outputWriter.flush();
            logger.debug("Sent command to OSland: " + command + " with data: " + json);
        } catch (IOException e) {
            logger.error("Error sending command to OSland", e);
            if (messageListener != null) {
                messageListener.onError("Failed to send command to OSland: " + e.getMessage());
            }
        } catch (Exception e) {
            logger.error("Unexpected error when sending command to OSland", e);
            if (messageListener != null) {
                messageListener.onError("Unexpected error: " + e.getMessage());
            }
        }
    }

    public void setMessageListener(MessageListener listener) {
        this.messageListener = listener;
    }

    public boolean isOSlandRunning() {
        return oslandProcess != null && oslandProcess.isAlive();
    }

    public static OSlandCommunicationService getInstance() {
        if (instance == null) {
            instance = ServiceManager.getService(OSlandCommunicationService.class);
        }
        return instance;
    }

    private static class CommandMessage {
        private String command;
        private Object data;

        public CommandMessage(String command, Object data) {
            this.command = command;
            this.data = data;
        }

        public String getCommand() {
            return command;
        }

        public void setCommand(String command) {
            this.command = command;
        }

        public Object getData() {
            return data;
        }

        public void setData(Object data) {
            this.data = data;
        }
    }
}
