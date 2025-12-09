package com.osland.plugin.actions;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.osland.plugin.OSlandCommunicationService;

public class RunProjectAction extends AnAction {
    @Override
    public void actionPerformed(AnActionEvent e) {
        OSlandCommunicationService communicationService = OSlandCommunicationService.getInstance();
        communicationService.sendCommand("run-project", null);
    }
}
