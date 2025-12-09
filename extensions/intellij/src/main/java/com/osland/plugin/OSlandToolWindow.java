package com.osland.plugin;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.wm.ToolWindow;
import com.intellij.ui.components.JBPanel;
import com.intellij.ui.components.JBScrollPane;
import com.intellij.ui.treeStructure.Tree;
import com.intellij.util.ui.JBUI;
import org.jetbrains.annotations.NotNull;

import javax.swing.*;
import javax.swing.tree.DefaultMutableTreeNode;
import javax.swing.tree.DefaultTreeModel;
import java.awt.*;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.awt.event.MouseMotionAdapter;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class OSlandToolWindow {
    private final Project project;
    private final JPanel contentPanel;
    private final CanvasPanel canvasPanel;
    private final JTree componentTree;
    private final JBPanel propertyPanel;
    private final OSlandCommunicationService communicationService;
    private final ObjectMapper objectMapper = new ObjectMapper();
    private ComponentNode selectedComponent;
    private Node selectedNode;

    public OSlandToolWindow(@NotNull Project project) {
        this.project = project;
        this.communicationService = OSlandCommunicationService.getInstance();
        this.contentPanel = new JPanel(new BorderLayout());
        this.canvasPanel = new CanvasPanel();
        this.componentTree = createComponentTree();
        this.propertyPanel = new JBPanel<>();
        this.selectedComponent = null;
        this.selectedNode = null;

        initUI();
        initCommunication();
    }

    private void initUI() {
        // Main split pane
        JSplitPane mainSplitPane = new JSplitPane(JSplitPane.HORIZONTAL_SPLIT);
        mainSplitPane.setDividerLocation(250);

        // Left split pane (components and canvas)
        JSplitPane leftSplitPane = new JSplitPane(JSplitPane.VERTICAL_SPLIT);
        leftSplitPane.setDividerLocation(300);

        // Component panel
        JBScrollPane componentScrollPane = new JBScrollPane(componentTree);
        componentScrollPane.setBorder(BorderFactory.createTitledBorder("Components"));

        // Canvas panel
        canvasPanel.setBorder(BorderFactory.createTitledBorder("Canvas"));
        JBScrollPane canvasScrollPane = new JBScrollPane(canvasPanel);

        // Property panel
        propertyPanel.setBorder(BorderFactory.createTitledBorder("Properties"));
        propertyPanel.setLayout(new BoxLayout(propertyPanel, BoxLayout.Y_AXIS));

        // Add components to split panes
        leftSplitPane.setLeftComponent(componentScrollPane);
        leftSplitPane.setRightComponent(canvasScrollPane);

        mainSplitPane.setLeftComponent(leftSplitPane);
        mainSplitPane.setRightComponent(propertyPanel);

        contentPanel.add(mainSplitPane, BorderLayout.CENTER);
    }

    private JTree createComponentTree() {
        // Create root node
        DefaultMutableTreeNode root = new DefaultMutableTreeNode("OSland Components");

        // Create component categories
        DefaultMutableTreeNode processors = new DefaultMutableTreeNode("Processors");
        DefaultMutableTreeNode memory = new DefaultMutableTreeNode("Memory");
        DefaultMutableTreeNode storage = new DefaultMutableTreeNode("Storage");
        DefaultMutableTreeNode network = new DefaultMutableTreeNode("Network");
        DefaultMutableTreeNode peripherals = new DefaultMutableTreeNode("Peripherals");
        DefaultMutableTreeNode kernel = new DefaultMutableTreeNode("Kernel");

        // Add components to categories
        processors.add(new DefaultMutableTreeNode(new ComponentNode("cpu", "CPU", "processor")));
        processors.add(new DefaultMutableTreeNode(new ComponentNode("gpu", "GPU", "processor")));
        memory.add(new DefaultMutableTreeNode(new ComponentNode("ram", "RAM", "memory")));
        memory.add(new DefaultMutableTreeNode(new ComponentNode("rom", "ROM", "memory")));
        storage.add(new DefaultMutableTreeNode(new ComponentNode("hard_disk", "Hard Disk", "storage")));
        storage.add(new DefaultMutableTreeNode(new ComponentNode("ssd", "SSD", "storage")));
        network.add(new DefaultMutableTreeNode(new ComponentNode("nic", "Network Card", "network")));
        network.add(new DefaultMutableTreeNode(new ComponentNode("router", "Router", "network")));
        peripherals.add(new DefaultMutableTreeNode(new ComponentNode("keyboard", "Keyboard", "peripheral")));
        peripherals.add(new DefaultMutableTreeNode(new ComponentNode("mouse", "Mouse", "peripheral")));
        peripherals.add(new DefaultMutableTreeNode(new ComponentNode("monitor", "Monitor", "peripheral")));
        kernel.add(new DefaultMutableTreeNode(new ComponentNode("kernel", "Kernel", "kernel")));
        kernel.add(new DefaultMutableTreeNode(new ComponentNode("scheduler", "Scheduler", "kernel")));
        kernel.add(new DefaultMutableTreeNode(new ComponentNode("memory_manager", "Memory Manager", "kernel")));
        kernel.add(new DefaultMutableTreeNode(new ComponentNode("file_system", "File System", "kernel")));
        kernel.add(new DefaultMutableTreeNode(new ComponentNode("driver", "Driver", "kernel")));

        // Add categories to root
        root.add(processors);
        root.add(memory);
        root.add(storage);
        root.add(network);
        root.add(peripherals);
        root.add(kernel);

        // Create tree
        JTree tree = new Tree(new DefaultTreeModel(root));
        tree.setCellRenderer(new ComponentTreeCellRenderer());
        tree.addTreeSelectionListener(e -> {
            DefaultMutableTreeNode selectedNode = (DefaultMutableTreeNode) tree.getLastSelectedPathComponent();
            if (selectedNode != null && selectedNode.getUserObject() instanceof ComponentNode) {
                selectedComponent = (ComponentNode) selectedNode.getUserObject();
                // Handle component selection
                communicationService.sendCommand("component-selected", selectedComponent);
            } else {
                selectedComponent = null;
            }
        });

        return tree;
    }

    private void initCommunication() {
        communicationService.setMessageListener(new OSlandCommunicationService.MessageListener() {
            @Override
            public void onMessage(String message) {
                // Handle messages from OSland
                SwingUtilities.invokeLater(() -> {
                    try {
                        JsonNode jsonNode = objectMapper.readTree(message);
                        String command = jsonNode.get("command").asText();
                        JsonNode data = jsonNode.get("data");

                        switch (command) {
                            case "node-selected":
                                handleNodeSelected(data);
                                break;
                            case "property-changed":
                                handlePropertyChanged(data);
                                break;
                            case "canvas-updated":
                                handleCanvasUpdated(data);
                                break;
                            default:
                                break;
                        }
                    } catch (Exception e) {
                        e.printStackTrace();
                    }
                });
            }

            @Override
            public void onError(String error) {
                // Handle errors
                SwingUtilities.invokeLater(() -> {
                    JOptionPane.showMessageDialog(contentPanel, error, "OSland Error", JOptionPane.ERROR_MESSAGE);
                });
            }
        });

        // Start OSland if not running
        if (!communicationService.isOSlandRunning()) {
            communicationService.startOSland();
        }
    }

    private void handleNodeSelected(JsonNode data) {
        // Handle node selected message from OSland
        String nodeId = data.get("id").asText();
        selectedNode = canvasPanel.findNodeById(nodeId);
        if (selectedNode != null) {
            updatePropertyPanel(selectedNode);
        }
    }

    private void handlePropertyChanged(JsonNode data) {
        // Handle property changed message from OSland
        String nodeId = data.get("id").asText();
        String property = data.get("property").asText();
        String value = data.get("value").asText();

        Node node = canvasPanel.findNodeById(nodeId);
        if (node != null) {
            node.getProperties().put(property, value);
            canvasPanel.repaint();
            updatePropertyPanel(node);
        }
    }

    private void handleCanvasUpdated(JsonNode data) {
        // Handle canvas updated message from OSland
        // This would parse the data and update the canvas
        canvasPanel.repaint();
    }

    private void updatePropertyPanel(Node node) {
        propertyPanel.removeAll();
        propertyPanel.setLayout(new BoxLayout(propertyPanel, BoxLayout.Y_AXIS));

        // Add node properties
        JLabel titleLabel = new JLabel("Node Properties");
        titleLabel.setFont(new Font(Font.DIALOG, Font.BOLD, 14));
        propertyPanel.add(titleLabel);
        propertyPanel.add(Box.createVerticalStrut(10));

        // Add id property
        propertyPanel.add(new JLabel("ID: " + node.getId()));
        propertyPanel.add(Box.createVerticalStrut(5));

        // Add name property
        propertyPanel.add(new JLabel("Name: " + node.getName()));
        propertyPanel.add(Box.createVerticalStrut(5));

        // Add type property
        propertyPanel.add(new JLabel("Type: " + node.getType()));
        propertyPanel.add(Box.createVerticalStrut(10));

        // Add other properties
        for (Map.Entry<String, String> entry : node.getProperties().entrySet()) {
            JPanel propertyRow = new JPanel(new BorderLayout());
            propertyRow.add(new JLabel(entry.getKey() + ": "), BorderLayout.WEST);
            JTextField textField = new JTextField(entry.getValue());
            textField.addActionListener(e -> {
                String value = textField.getText();
                node.getProperties().put(entry.getKey(), value);
                // Send property changed message to OSland
                communicationService.sendCommand("property-changed", Map.of(
                        "id", node.getId(),
                        "property", entry.getKey(),
                        "value", value
                ));
            });
            propertyRow.add(textField, BorderLayout.CENTER);
            propertyPanel.add(propertyRow);
            propertyPanel.add(Box.createVerticalStrut(5));
        }

        propertyPanel.revalidate();
        propertyPanel.repaint();
    }

    public JPanel getContentPanel() {
        return contentPanel;
    }

    // Node class for canvas
    private static class Node {
        private final String id;
        private final String name;
        private final String type;
        private final Map<String, String> properties;
        private int x;
        private int y;

        public Node(String id, String name, String type, int x, int y) {
            this.id = id;
            this.name = name;
            this.type = type;
            this.properties = new HashMap<>();
            this.x = x;
            this.y = y;
        }

        public String getId() {
            return id;
        }

        public String getName() {
            return name;
        }

        public String getType() {
            return type;
        }

        public Map<String, String> getProperties() {
            return properties;
        }

        public int getX() {
            return x;
        }

        public int getY() {
            return y;
        }

        public void setX(int x) {
            this.x = x;
        }

        public void setY(int y) {
            this.y = y;
        }
    }

    // Connection class for canvas
    private static class Connection {
        private final String sourceNodeId;
        private final String sourcePortId;
        private final String targetNodeId;
        private final String targetPortId;

        public Connection(String sourceNodeId, String sourcePortId, String targetNodeId, String targetPortId) {
            this.sourceNodeId = sourceNodeId;
            this.sourcePortId = sourcePortId;
            this.targetNodeId = targetNodeId;
            this.targetPortId = targetPortId;
        }

        public String getSourceNodeId() {
            return sourceNodeId;
        }

        public String getSourcePortId() {
            return sourcePortId;
        }

        public String getTargetNodeId() {
            return targetNodeId;
        }

        public String getTargetPortId() {
            return targetPortId;
        }
    }

    // Canvas panel with drag and drop support
    private class CanvasPanel extends JBPanel {
        private static final int NODE_WIDTH = 120;
        private static final int NODE_HEIGHT = 80;
        private static final int PORT_RADIUS = 5;

        private final List<Node> nodes = new ArrayList<>();
        private final List<Connection> connections = new ArrayList<>();
        private Node draggingNode;
        private Point dragOffset;
        private int nextNodeId = 1;

        public CanvasPanel() {
            setBackground(Color.WHITE);
            setLayout(null);

            // Add mouse listeners for drag and drop
            addMouseListener(new MouseAdapter() {
                @Override
                public void mousePressed(MouseEvent e) {
                    if (selectedComponent != null) {
                        // Add new node at click location
                        String nodeId = "node-" + nextNodeId++;
                        Node newNode = new Node(nodeId, selectedComponent.getName(), selectedComponent.getType(), e.getX() - NODE_WIDTH / 2, e.getY() - NODE_HEIGHT / 2);
                        nodes.add(newNode);
                        repaint();

                        // Send component added message to OSland
                        communicationService.sendCommand("component-added", Map.of(
                                "id", newNode.getId(),
                                "name", newNode.getName(),
                                "type", newNode.getType(),
                                "x", newNode.getX(),
                                "y", newNode.getY()
                        ));
                    } else {
                        // Check if clicking on a node
                        draggingNode = findNodeAt(e.getX(), e.getY());
                        if (draggingNode != null) {
                            dragOffset = new Point(e.getX() - draggingNode.getX(), e.getY() - draggingNode.getY());
                            selectedNode = draggingNode;
                            updatePropertyPanel(draggingNode);
                            // Send node selected message to OSland
                            communicationService.sendCommand("node-selected", Map.of(
                                    "id", draggingNode.getId()
                            ));
                        } else {
                            selectedNode = null;
                            propertyPanel.removeAll();
                            propertyPanel.revalidate();
                            propertyPanel.repaint();
                        }
                    }
                    repaint();
                }

                @Override
                public void mouseReleased(MouseEvent e) {
                    draggingNode = null;
                    dragOffset = null;
                }
            });

            addMouseMotionListener(new MouseMotionAdapter() {
                @Override
                public void mouseDragged(MouseEvent e) {
                    if (draggingNode != null && dragOffset != null) {
                        draggingNode.setX(e.getX() - dragOffset.x);
                        draggingNode.setY(e.getY() - dragOffset.y);
                        repaint();

                        // Send node moved message to OSland
                        communicationService.sendCommand("node-moved", Map.of(
                                "id", draggingNode.getId(),
                                "x", draggingNode.getX(),
                                "y", draggingNode.getY()
                        ));
                    }
                }
            });
        }

        private Node findNodeAt(int x, int y) {
            for (Node node : nodes) {
                if (x >= node.getX() && x <= node.getX() + NODE_WIDTH &&
                        y >= node.getY() && y <= node.getY() + NODE_HEIGHT) {
                    return node;
                }
            }
            return null;
        }

        public Node findNodeById(String id) {
            for (Node node : nodes) {
                if (node.getId().equals(id)) {
                    return node;
                }
            }
            return null;
        }

        @Override
        protected void paintComponent(Graphics g) {
            super.paintComponent(g);
            Graphics2D g2d = (Graphics2D) g;

            // Draw connections
            g2d.setColor(new Color(150, 150, 150));
            g2d.setStroke(new BasicStroke(2));
            for (Connection connection : connections) {
                Node sourceNode = findNodeById(connection.getSourceNodeId());
                Node targetNode = findNodeById(connection.getTargetNodeId());

                if (sourceNode != null && targetNode != null) {
                    // Draw line from source to target
                    int sourceX = sourceNode.getX() + NODE_WIDTH;
                    int sourceY = sourceNode.getY() + NODE_HEIGHT / 2;
                    int targetX = targetNode.getX();
                    int targetY = targetNode.getY() + NODE_HEIGHT / 2;

                    g2d.drawLine(sourceX, sourceY, targetX, targetY);

                    // Draw arrowhead
                    drawArrowHead(g2d, targetX, targetY, sourceX, sourceY);
                }
            }

            // Draw nodes
            Map<String, Color> typeColors = new HashMap<>();
            typeColors.put("processor", new Color(255, 159, 64));
            typeColors.put("memory", new Color(74, 144, 226));
            typeColors.put("storage", new Color(102, 192, 102));
            typeColors.put("network", new Color(192, 97, 203));
            typeColors.put("peripheral", new Color(226, 114, 128));
            typeColors.put("kernel", new Color(246, 201, 48));

            for (Node node : nodes) {
                Color color = typeColors.getOrDefault(node.getType(), Color.GRAY);
                boolean isSelected = node == selectedNode;

                // Draw node background
                if (isSelected) {
                    g2d.setColor(color.darker());
                } else {
                    g2d.setColor(color);
                }
                g2d.fillRect(node.getX(), node.getY(), NODE_WIDTH, NODE_HEIGHT);

                // Draw node border
                g2d.setColor(Color.BLACK);
                g2d.setStroke(new BasicStroke(2));
                g2d.drawRect(node.getX(), node.getY(), NODE_WIDTH, NODE_HEIGHT);

                // Draw node name
                g2d.setColor(Color.WHITE);
                g2d.setFont(new Font(Font.DIALOG, Font.BOLD, 14));
                FontMetrics metrics = g2d.getFontMetrics();
                int textWidth = metrics.stringWidth(node.getName());
                int textX = node.getX() + (NODE_WIDTH - textWidth) / 2;
                int textY = node.getY() + (NODE_HEIGHT + metrics.getAscent()) / 2 - metrics.getDescent();
                g2d.drawString(node.getName(), textX, textY);

                // Draw ports
                g2d.setColor(Color.WHITE);
                // Input port (left)
                g2d.fillOval(node.getX() - PORT_RADIUS, node.getY() + NODE_HEIGHT / 2 - PORT_RADIUS, PORT_RADIUS * 2, PORT_RADIUS * 2);
                // Output port (right)
                g2d.fillOval(node.getX() + NODE_WIDTH - PORT_RADIUS, node.getY() + NODE_HEIGHT / 2 - PORT_RADIUS, PORT_RADIUS * 2, PORT_RADIUS * 2);
            }
        }

        private void drawArrowHead(Graphics2D g2d, int x1, int y1, int x2, int y2) {
            int arrowSize = 10;
            double angle = Math.atan2(y1 - y2, x1 - x2);
            g2d.setStroke(new BasicStroke(1));

            int x3 = (int) (x1 - arrowSize * Math.cos(angle - Math.PI / 6));
            int y3 = (int) (y1 - arrowSize * Math.sin(angle - Math.PI / 6));
            int x4 = (int) (x1 - arrowSize * Math.cos(angle + Math.PI / 6));
            int y4 = (int) (y1 - arrowSize * Math.sin(angle + Math.PI / 6));

            g2d.drawLine(x1, y1, x3, y3);
            g2d.drawLine(x1, y1, x4, y4);
        }
    }

    // Component node class
    private static class ComponentNode {
        private final String id;
        private final String name;
        private final String type;

        public ComponentNode(String id, String name, String type) {
            this.id = id;
            this.name = name;
            this.type = type;
        }

        public String getId() {
            return id;
        }

        public String getName() {
            return name;
        }

        public String getType() {
            return type;
        }

        @Override
        public String toString() {
            return name;
        }
    }

    // Tree cell renderer
    private static class ComponentTreeCellRenderer extends javax.swing.tree.DefaultTreeCellRenderer {
        private final Map<String, Color> typeColors = new HashMap<>();

        public ComponentTreeCellRenderer() {
            typeColors.put("processor", new Color(255, 159, 64));
            typeColors.put("memory", new Color(74, 144, 226));
            typeColors.put("storage", new Color(102, 192, 102));
            typeColors.put("network", new Color(192, 97, 203));
            typeColors.put("peripheral", new Color(226, 114, 128));
            typeColors.put("kernel", new Color(246, 201, 48));
        }

        @Override
        public Component getTreeCellRendererComponent(JTree tree, Object value, boolean selected, boolean expanded, boolean leaf, int row, boolean hasFocus) {
            super.getTreeCellRendererComponent(tree, value, selected, expanded, leaf, row, hasFocus);

            DefaultMutableTreeNode node = (DefaultMutableTreeNode) value;
            Object userObject = node.getUserObject();

            if (userObject instanceof ComponentNode) {
                ComponentNode componentNode = (ComponentNode) userObject;
                Color color = typeColors.getOrDefault(componentNode.getType(), Color.BLACK);
                setForeground(color);
            }

            return this;
        }
    }
}
