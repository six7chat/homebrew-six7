class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.6"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.6"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.6/six7-0.7.6-arm64-apple-darwin.tar.gz"
      sha256 "8639e3d88d3886e873c93c854ec2b9518a7ec6bf094132291a8de1c133f9a4e1"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
