package com.osland.plugin.actions;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.fileChooser.FileChooser;
import com.intellij.openapi.fileChooser.FileChooserDescriptor;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.vfs.VirtualFile;
import com.osland.plugin.OSlandCommunicationService;

public class LoadProjectAction extends AnAction {
    @Override
    public void actionPerformed(AnActionEvent e) {
        Project project = e.getProject();
        if (project != null) {
            // Show file chooser dialog
            FileChooserDescriptor descriptor = new FileChooserDescriptor(
                    false, true, false, false, false, false
            );
            descriptor.setTitle("Load OSland Project");
            descriptor.setDescription("Choose a directory containing the OSland project");

            VirtualFile file = FileChooser.chooseFile(descriptor, project, null);
            if (file != null) {
                String path = file.getPath();
                OSlandCommunicationService communicationService = OSlandCommunicationService.getInstance();
                communicationService.sendCommand("load-project", path);
            }
        }
    }
}
