class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.8"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.8"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.8/six7-0.7.8-arm64-apple-darwin.tar.gz"
      sha256 "69eb431bc7af553fd0ea19720cf22a7f22d7539bf4f1118028db1ca76e1e374a"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
