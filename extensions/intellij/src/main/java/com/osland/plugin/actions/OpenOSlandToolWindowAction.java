package com.osland.plugin.actions;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.wm.ToolWindowManager;
import com.osland.plugin.OSlandCommunicationService;

public class OpenOSlandToolWindowAction extends AnAction {
    @Override
    public void actionPerformed(AnActionEvent e) {
        Project project = e.getProject();
        if (project != null) {
            // Open the OSland tool window
            ToolWindowManager.getInstance(project).getToolWindow("OSland").show(null);

            // Start OSland if not running
            OSlandCommunicationService communicationService = OSlandCommunicationService.getInstance();
            if (!communicationService.isOSlandRunning()) {
                communicationService.startOSland();
            }
        }
    }
}
